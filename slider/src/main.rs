use macroquad::{color::{BLACK, BLUE, WHITE}, shapes::draw_rectangle, window::{clear_background, next_frame, screen_width}};
use macroquad::prelude::*;



struct Slider {
    pos_x_barra: f32,
    pos_y_barra: f32,
    ancho_barra: f32,
    alto: f32,
    ancho_slider: f32,
    pos_x_slider: f32,
    color_barra: Color,
    color_slider: Color,
    arrastrando: bool,
    nombre : String,
}

impl Slider {

    pub fn nuevo_slider(nombre: &str, pos_y: f32, ancho: f32, alto: f32, valor: f32, color_barra: Color, color_slider: Color) -> Self {
        let x: f32 = (screen_width() / 2.0) - (ancho / 2.0);
        
        Self{
            pos_x_barra: x,
            pos_y_barra: pos_y,
            ancho_barra: ancho,
            alto: alto,
            ancho_slider: 15.0,
            pos_x_slider: valor,
            color_barra: color_barra,
            color_slider: color_slider,
            arrastrando: false,
            nombre: nombre.to_string(),
        }
    }

    pub fn posicion_mouse() -> (f32, f32) {
        let (raton_x, raton_y) = mouse_position();
        
        (raton_x, raton_y)
    }

    pub fn posicionar_slider(&self, valor: f32) -> f32 {
        let ancho_util_barra: f32 = self.ancho_barra - self.ancho_slider;
        let x_slider = self.pos_x_barra + (valor * ancho_util_barra);
        
        x_slider
    }

    pub fn mover_slider(&mut self, valor: f32) -> f32 {
    let (x_mouse, y_mouse) = Slider::posicion_mouse();
    
    // Obtener la posición del slider sincronizada con el main
    self.pos_x_slider = self.posicionar_slider(valor);

    // Si no se presiona el clic, soltamos el slider
    if !is_mouse_button_down(MouseButton::Left) {
        self.arrastrando = false;
        return valor;
    } 

    if x_mouse >= self.pos_x_slider && x_mouse <= (self.pos_x_slider + self.ancho_slider) {
        if y_mouse >= self.pos_y_barra && y_mouse <= self.pos_y_barra + self.alto {
            self.arrastrando = true;
        }
    }
    

    // Si ya estamos arrastrando el cursor
    if self.arrastrando {
        let ancho_util_barra = self.ancho_barra - self.ancho_slider;
        let click_relativo_x = x_mouse - self.pos_x_barra - (self.ancho_slider / 2.0);
        let nuevo = click_relativo_x / ancho_util_barra;
        let nuevo_clamped = nuevo.clamp(0.0, 1.0);
        
        // Actualizar la nueva posición del slider
        self.pos_x_slider = self.posicionar_slider(nuevo_clamped);
        return nuevo_clamped; 
    }

    return valor;
    }



    pub fn pintar_slider(&self, valor: f32) {
        let x_slider = self.posicionar_slider(valor);

        let texto_eqtiqueta = format!("{}: {:.0}%", self.nombre, valor * 100.0);
        let pos_y_eqtiqueta = self.pos_y_barra - 10.0;
        let pos_x_etiqueta = self.ancho_barra / 2.0;
        //Etiqueta
        draw_text(&texto_eqtiqueta, pos_x_etiqueta, pos_y_eqtiqueta, 24.0, self.color_slider);
        //Barra
        draw_rectangle(self.pos_x_barra, self.pos_y_barra, self.ancho_barra, self.alto, self.color_barra);
        //Slider
        draw_rectangle(x_slider, self.pos_y_barra, self.ancho_slider, self.alto, self.color_slider);
    }

}

#[macroquad::main("Slider")]
async fn main() {
    println!("Hello, world!");
    let mut valor1: f32 = 0.5;
    let mut slider1 = Slider::nuevo_slider("Volumen",50.0, 400.0, 20.0, valor1, WHITE, BLACK);
    slider1.pos_x_slider = slider1.posicionar_slider(valor1);
    //let slider2 = Slider::nuevo_slider(100.0, 400.0, 20.0, valor1, WHITE, BLACK);

    loop {
        clear_background(BLUE);
        //let mut direccion_arrastre: DireccionRaton = DireccionRaton::None;
        //let valor: f32 = 1.0;

       
        slider1.pintar_slider(valor1);       
        valor1 = slider1.mover_slider(valor1);
        println!("Volumen: {}%", valor1 * 100.0);
        //slider1.posicionar_slider(valor1);
        
        //slider2.pintar_slider(valor1);
/* 
        if is_mouse_button_down(MouseButton::Left) {
            let (raton_x, raton_y) = mouse_position();
        }
            */

        next_frame().await;
    }
}
