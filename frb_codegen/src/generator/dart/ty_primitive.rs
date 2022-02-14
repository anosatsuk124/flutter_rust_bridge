use crate::generator::dart::ty::TypeDartGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypePrimitiveGenerator(IrTypePrimitive);

impl TypeDartGeneratorTrait for TypePrimitiveGenerator {
    fn api2wire_body(&self) -> String {
        match self.0 {
            IrTypePrimitive::Bool => "return raw ? 1 : 0;".to_owned(),
            _ => "return raw;".to_string(),
        }
    }

    fn wire2api_body(&self) -> String {
        match self.0 {
            IrTypePrimitive::Unit => "return;".to_owned(),
            _ => gen_wire2api_simple_type_cast(&p.dart_api_type()),
        }
    }
}
