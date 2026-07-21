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
use uln2003::StepperMotor;

use crate::module::{Module};
use embassy_sync::mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;

use esp_hal::gpio::{Output,Input};

pub struct AppState<'a> {
    pub character: &'a Mutex<NoopRawMutex, Module<'a>>,
    pub spawner: embassy_executor::Spawner
}
#[derive(Deserialize)]
pub struct CharForm
{
    pub val: String<100>
}

async fn character_control_handler(State(state): State<&AppState<'_>>,Form(form_data) : Form<CharForm>) -> impl IntoResponse {
    let mut character_locked = state.character.lock().await;
    for c in form_data.val.as_bytes()
    {
        character_locked.print_char(*c);
        embassy_time::Timer::after(embassy_time::Duration::from_millis(500)).await;
    }
    //state.spawner.spawn(print_word_task(state,form_data.val).unwrap());
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
        .route("/reset", routing::get(async move |State(state): State<&AppState<'_>>| {state.character.lock().await.reset();""} ))
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
pub async fn setup_character_controller_server(motor_outs: (Output<'static>,Output<'static>,Output<'static>,Output<'static>), hall_sensor: Input<'static>, stack: Stack<'static>, spawner: Spawner)
{
    let mut motor = uln2003::ULN2003::new(motor_outs.0, motor_outs.1, motor_outs.2, motor_outs.3, Some(embassy_time::Delay));
    motor.set_direction(uln2003::Direction::Reverse);
    let character = crate::module::Module::new(37,motor,hall_sensor);
    let character_mutex= make_static!(Mutex<NoopRawMutex, Module<'_>>,Mutex::new(character));
    let state = make_static!(AppState<'static>, AppState { character: character_mutex, spawner: spawner });
    let app =  make_static!(WebApp,crate::web::WebApp::new(state));
    spawner.spawn(crate::web::web_task(0, stack, app.router, app.config).unwrap());
    spawner.spawn(crate::web::web_task(0, stack, app.router, app.config).unwrap());
}


#[embassy_executor::task]
pub async fn reset_task(character: &'static mut Module<'static>)
{
    character.reset();
} 
#[embassy_executor::task]
pub async fn print_word_task(character_mutex: &'static embassy_sync::mutex::Mutex<NoopRawMutex, Module<'static>>,word: String<100>)
{
    let mut character = character_mutex.lock().await;
    for c in word.as_bytes()
    {
        character.print_char(*c);
        embassy_time::Timer::after(embassy_time::Duration::from_millis(500)).await;
    }
}