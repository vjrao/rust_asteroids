use super::Vertex;

pub trait Shape {
    fn vertices(&self, vertex_list : &mut Vec<Vertex>);

    fn indices(index_list : &mut Vec<u16>, offset : u16);

    fn still_alive(&self) -> bool;
}
