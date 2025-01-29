use std::sync::OnceLock;

// Déclaration de la variable globale FRAME_RATE
static FRAME_RATE: OnceLock<usize> = OnceLock::new();

#[derive(Debug, Clone)]
struct Marque {
    forme: String,
    position: (f32, f32),
    couleur: (u8, u8, u8),
}

impl Marque {
    fn interpolate_float(start: f32, end: f32, t: f32) -> f32 {
        start + (end - start) * t
    }

    fn interpolate_color(start: (u8, u8, u8), end: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
        (
            (start.0 as f32 + (end.0 as f32 - start.0 as f32) * t) as u8,
            (start.1 as f32 + (end.1 as f32 - start.1 as f32) * t) as u8,
            (start.2 as f32 + (end.2 as f32 - start.2 as f32) * t) as u8,
        )
    }
}


fn interpolate<F>(func: F, start: &Marque, end: &Marque) -> Vec<Marque> 
    where F: Fn(f32) -> f32 {
    let frame_rate = *FRAME_RATE.get_or_init(|| 60); 
    let mut result = Vec::with_capacity(frame_rate);


    //rajouter une ligne pour permettre à F d'imfluencer la variation des vars
    for i in 0..frame_rate {
        let t = func(i as f32 / (frame_rate as f32 - 1.0));
        result.push(Marque {
            forme: start.forme.clone(),
            position: (
                Marque::interpolate_float(start.position.0, end.position.0, t),
                Marque::interpolate_float(start.position.1, end.position.1, t),
            ),
            couleur: Marque::interpolate_color(start.couleur, end.couleur, t),
        });
    }

    result
}

fn main() {
    FRAME_RATE.set(30).expect("FRAME_RATE déjà initialisé");

    let start = Marque {
        forme: "cercle".to_string(),
        position: (0.0, 0.0),
        couleur: (255, 0, 0),
    };

    let end = Marque {
        forme: "cercle".to_string(),
        position: (100.0, 100.0),
        couleur: (0, 255, 0),
    };

    //exemple de fct d'interpolation
    //Hassoul elle est pas linéaire mais osef 
    let ease_in_out = |t: f32| if t < 0.5 { 2.0 * t * t } else { -1.0 + (4.0 - 2.0 * t) * t };

    let interpolated_frames = interpolate(ease_in_out, &start, &end);

    for (i, frame) in interpolated_frames.iter().enumerate() {
        println!("Frame {}: {:?}", i, frame);
    }
}
