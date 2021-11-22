use super::*;
use crate::shape::{Shape, Shapes};
use std::marker::PhantomData;

pub struct DrawShape<RenderType, ShapeType>
where
    ShapeType: Shape<RenderType>,
{
    name: String,
    shape: ShapeType,
    phantom: PhantomData<RenderType>,
}

impl<RenderType, ShapeType> DrawShape<RenderType, ShapeType>
where
    ShapeType: Shape<RenderType>,
{
    pub fn new(name: String, shape: ShapeType) -> Self {
        Self {
            name,
            shape,
            phantom: PhantomData,
        }
    }
}

impl<RenderType, ShapeType> Default for DrawShape<RenderType, ShapeType>
where
    ShapeType: Shape<RenderType> + Default,
{
    fn default() -> Self {
        Self {
            name: std::any::type_name::<ShapeType>().into(),
            shape: ShapeType::default(),
            phantom: PhantomData,
        }
    }
}

impl<RenderType, ShapeType> fmt::Display for DrawShape<RenderType, ShapeType>
where
    ShapeType: Shape<RenderType>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.name, self.shape)
    }
}

impl<RenderType, ShapeType> Command<RenderType> for DrawShape<RenderType, ShapeType>
where
    ShapeType: 'static + Shape<RenderType> + Copy,
{
    fn execute(&self, shapes: &mut Shapes<RenderType>) -> Result<(), Box<dyn Error>> {
        shapes.insert(self.name.clone(), Box::new((self.shape).clone()));
        Ok(())
    }
    fn undo(&self, shapes: &mut Shapes<RenderType>) -> Result<(), Box<dyn Error>> {
        shapes.remove(&self.name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::DummyRenderer;
    use crate::shape::*;

    #[test]
    fn execute() {
        let cmd1 = DrawShape::<DummyRenderer, _> {
            shape: Point::default(),
            name: "p1".to_string(),
            phantom: PhantomData,
        };
        let cmd2 = DrawShape::<DummyRenderer, _> {
            shape: Rectangle::default(),
            name: "p2".to_string(),
            phantom: PhantomData,
        };
        let mut shapes = Shapes::default();

        cmd1.execute(&mut shapes).unwrap();
        assert_eq!(
            format!("{:?}", shapes[&cmd1.name]),
            format!("{:?}", cmd1.shape),
        );

        cmd1.undo(&mut shapes).unwrap();
        assert!(shapes.get(&cmd1.name).is_none());
        assert_eq!(shapes.len(), 0);

        cmd1.execute(&mut shapes).unwrap();
        cmd2.execute(&mut shapes).unwrap();
        assert_eq!(
            format!("{:?}", shapes[&cmd1.name]),
            format!("{:?}", cmd1.shape),
        );
        assert_eq!(
            format!("{:?}", shapes[&cmd2.name]),
            format!("{:?}", cmd2.shape),
        );

        cmd1.undo(&mut shapes).unwrap();
        assert_eq!(shapes.len(), 1);
        assert_eq!(
            format!("{:?}", shapes[&cmd2.name]),
            format!("{:?}", cmd2.shape),
        );

        cmd2.undo(&mut shapes).unwrap();
        assert_eq!(shapes.len(), 0);
    }
}
