struct RenderElementTree {
    label: &'static str,
    render_elements: HashMap<u64, RenderElement>,
    attachments: Attachments,
}
impl RenderElementTree {
    fn new(label: &'static str) -> Self {
        Self {
            label,
            render_elements: HashMap::<u64, RenderElement>::new(),
            attachments: Attachments::new(),
        }
    }
    fn insert(&mut self, element: RenderElement) -> Result<u64, String> {
        for i in 0..u64::MAX {
            match self.render_elements.get(&i) {
                Some(r) => {
                        // go on
                    },
                None => {
                    self.render_elements.insert(i, element);
                    return Ok(i);
                },
        
            }
        }
        let label = self.label;
        let message = format!("failed to insert a RenderElement into a RenderElementTree label: {label}\n probable cause: keys up to u64::MAX taken" );
        Err(message)
        
    }
}

struct RenderElement {
    
}
impl RenderElement {
    
}


fn insert_render_element(&mut self, render_element: RenderElement) -> u64 {
        return self.tree.insert(render_element).unwrap();
    }