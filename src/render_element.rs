pub(self)

struct RenderElement {
    
}
impl RenderElement {
    
}

#[derive(PartialEq)]
pub enum Operation {
    Overwrite{
        commands: Vec<Operation>
    },
    Blend {
        layers: Vec<Operation>,
    },
    Circle {
        radius: f32,
        transform: glam::Mat3,
    }
}



