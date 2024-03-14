pub(self)

struct RenderElement {
    
}
impl RenderElement {
    
}

#[derive(PartialEq)]
pub enum Node {
    Overwrite{
        commands: Vec<Node>
    },
    Blend {
        layers: Vec<Node>,
    },
    Circle {
        radius: f32,
        transform: glam::Mat3,
    }
}



