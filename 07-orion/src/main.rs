use druid::{AppLauncher, WindowDesc};
use gui::{build_ui, AppData};

mod gui; // Módulo para la interfaz gráfica
mod http_client; // Módulo para las peticiones HTTP
mod freya; // gui freya


fn main() {
    let main_window = WindowDesc::new(build_ui())
        .title("Cliente HTTP")
        .window_size((500.0, 400.0));

    let initial_state = AppData{
        url: String::new(),
        method: String::new(),
        body: String::new(),
        response: String::new(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Error al lanzar la aplicación"); 

    //freya::app().run();
    freya::app().run();
    
}

