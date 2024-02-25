

struct RenderElement {
    
}
impl RenderElement {
    
}

enum Operation {
    Overwrite{
        commands: Vec<Operation>
    },
    Blend {
        layers: Vec<Operation>,
    }
}

fn test() {
    let elements = Operation::Blend {
        layers: vec![
            Operation::Overwrite{
                commands: Vec::new(),
            }
        ],
    };
}