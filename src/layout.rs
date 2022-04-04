struct Vertex {
    node_a: String,
    node_b: String,
    cost: u32,
}
pub struct Layout {
    vertices : Vec<Vertex>
}

impl Layout {
    pub fn new(data:Vec<&str>) -> Layout {
        let v:Vec<Vertex> = data.into_iter().map(|d| {
            let mut iter = d.split(":");

            Vertex{
                node_a: String::from(iter.next().unwrap()), 
                node_b: String::from(iter.next().unwrap()), 
                cost: iter.next().unwrap().parse::<u32>().unwrap()
            }
        }).collect(); 

        Layout {
            vertices: v
        }
    }
}


 #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_parse() {
        let v = vec!["a:b:6"];         
        let l = Layout::new(v);
        assert_eq!(l.vertices.len(), 1);
        assert_eq!(l.vertices[0].cost, 6);
        assert_eq!(l.vertices[0].node_b, "b");
        assert_eq!(l.vertices[0].node_a, "a");
    }
    
    #[test]
    fn full_data_parse() {
        let v = vec!["a:b:6","b:c:3" ];         
        let l = Layout::new(v);
        assert_eq!(l.vertices.len(), 2);
    }
}