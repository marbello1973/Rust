use druid::{Data, Lens, Widget, WidgetExt};
use druid::widget::{Button, Controller, Flex, Label, TextBox, Scroll}; // Importa los widgets desde el submódulo
use serde_json::Value;
use crate::http_client;

#[derive(Clone, Data, Lens)]
pub struct AppData {
    pub url: String,
    pub method: String,
    pub body: String,
    pub response: String,
}

struct ReadOnlyController;

impl <W: Widget<String>> Controller<String, W> for ReadOnlyController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut String,
        env: &druid::Env,
    ){
        match event {
            druid::Event::KeyDown(_) | druid::Event::Paste(_) => {
                // Ignora los eventos de teclado y pegado
            }
            _ => {
                child.event(ctx, event, data, env);
            }
        }
    }    
}


pub fn build_ui() -> impl Widget<AppData> {
    let url_input = TextBox::new()
        .with_placeholder("Ingrese la URL")
        .lens(AppData::url);

    let method_input = TextBox::new()
        .with_placeholder("GET, POST, PUT, DELETE")
        .lens(AppData::method);

    let body_input = TextBox::multiline()
        .with_placeholder("Cuerpo de la petición")
        .lens(AppData::body);

    let response_output = TextBox::multiline()
        .with_placeholder("Respuesta del servidor")
        .controller(ReadOnlyController)
        .lens(AppData::response)
        .expand_width(); 

    let scrollable_response = Scroll::new(response_output)
        .vertical()
        .expand_height()
        .fix_height(200.0);
      
    
    let send_button = Button::new("Enviar").on_click(|ctx, data: &mut AppData, _env| {

        if data.url.is_empty() {
			data.response = "URL no puede estar vacía".to_string();
			ctx.request_paint();
			return;
		}
                
        let response = http_client::send_request(&data.url, &data.method, Some(&data.body));
       
        match response {
            Ok(resp_text) => {
                data.response = if let Ok(json) = serde_json::from_str::<Value>(&resp_text) {
                    serde_json::to_string_pretty(&json).unwrap_or_else(|_| resp_text.clone())
                } else {
                    resp_text
                };
            }
            Err(err) => data.response = format!("Error: {}", err),
        }
        ctx.request_paint();
    });

    // Layout
    Flex::column()
        .with_child(Label::new("URL:"))
        .with_child(url_input)
        .with_child(Label::new("Método:"))
        .with_child(method_input)
        .with_child(Label::new("Cuerpo:"))
        .with_child(body_input)
        .with_child(send_button)
        .with_child(Label::new("Respuesta:"))        
        .with_child(scrollable_response)
}
