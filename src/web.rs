use embassy_executor::Spawner;
use esp_println::println;
use picoserve::{AppBuilder, make_static};
use picoserve::routing::{self,Router};
use picoserve::response::{File, Redirect};
use picoserve::time::Duration;
use picoserve::AppRouter;
use embassy_net::Stack;
use picoserve::extract::{State};
use picoserve::response::{IntoResponse};

use picoserve::extract::Form;
use serde::Deserialize;
use heapless::String;

use crate::character::{Character};
use embassy_sync::mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;

use esp_hal::gpio::{Output};

pub struct AppState<'a> {
    pub character: &'a Mutex<NoopRawMutex, Character<'a>>,
}
#[derive(Deserialize)]
pub struct CharForm
{
    pub val: String<10>,
    pub speed: u32
}

async fn character_control_handler(State(state): State<&AppState<'_>>,Form(form_data) : Form<CharForm>) -> impl IntoResponse {
    let mut char_data = state.character.lock().await;
    char_data.motor.set_speed(form_data.speed);
    for c in  form_data.val.as_bytes()
    {
        println!("Printing {c}");
        char_data.print_char(*c);
        embassy_time::Timer::after(embassy_time::Duration::from_millis(500)).await;
    }
    Redirect::to("/")
}
pub struct Application<'a>
{
    pub state: &'a AppState<'a>
}

impl AppBuilder for Application<'_> {
    type PathRouter = impl routing::PathRouter;

    fn build_app(self) -> picoserve::Router<Self::PathRouter> {
        picoserve::Router::new().route
        (
            "/print",
            routing::post(character_control_handler)
        )
        .route
        (
            "/",
            routing::get_service(File::html(include_str!("index.html")))
        )
        .route("/reset", routing::get(async move |State(state): State<&AppState<'_>>| {state.character.lock().await.position=0;""} ))
        .with_state(self.state)
    }
}
pub struct WebApp {
    pub router: &'static Router<<Application<'static> as AppBuilder>::PathRouter>,
    pub config: &'static picoserve::Config,
}

impl WebApp {
    pub fn new(state: &'static AppState) -> Self {
        
        let router = make_static!(AppRouter<Application>,Application {state}. build_app());
        let config = picoserve::make_static!(
            picoserve::Config,
            picoserve::Config::new(picoserve::Timeouts {
                start_read_request: Duration::from_secs(5),
                read_request: Duration::from_secs(1),
                write: Duration::from_secs(1),
                persistent_start_read_request: Duration::from_secs(1),
            })
            .keep_connection_alive()
        );

        Self { router, config }
    }
}

#[embassy_executor::task(pool_size = 2)]
pub async fn web_task(
    task_id: usize,
    stack: Stack<'static>,
    router: &'static AppRouter<Application<'static>>,
    config: &'static picoserve::Config,
) -> ! {
    println!("Starting web task...");
    let port = 80;
    let mut tcp_rx_buffer = [0; 1024];
    let mut tcp_tx_buffer = [0; 1024];
    let mut http_buffer = [0; 2048];

    picoserve::Server::new(router, config, &mut http_buffer)
        .listen_and_serve(task_id, stack, port, &mut tcp_rx_buffer, &mut tcp_tx_buffer)
        .await
        .into_never()
}

#[embassy_executor::task]
pub async fn setup_character_controller_server(motor_outs: (Output<'static>,Output<'static>,Output<'static>,Output<'static>), hall_sensor: Output<'static>, stack: Stack<'static>, spawner: Spawner)
{
    let mut motor = embedded_stepper::create_stepper_4pin(motor_outs.0, motor_outs.1, motor_outs.2, motor_outs.3, esp_hal::delay::Delay::new(), 2048);
    motor.set_speed(20);
    let character = crate::character::Character::new(37,motor,hall_sensor);
    let character_mutex= make_static!(Mutex<NoopRawMutex, Character<'_>>,Mutex::new(character));
    let state = make_static!(AppState<'static>, AppState { character: character_mutex });
    let app =  make_static!(WebApp,crate::web::WebApp::new(state));
    spawner.spawn(crate::web::web_task(0, stack, app.router, app.config).unwrap());
}
