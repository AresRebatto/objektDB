use super::impl_block::ImplBlock;
pub trait CallIfImpl{
    fn impl_block_trait_implemented()-> Result<Vec<String>, String>;
}

impl<T> CallIfImpl for T{
    default fn impl_block_trait_implemented()-> Result<Vec<String>, String>{
        return Err("ImplBlock trait is not implemented");
    }
}

impl<T:: ImplBlock> CallIfImpl for T{
    default fn impl_block_trait_implemented()-> Result<Vec<String>, String>{
        Ok(Self::get_methods_names())
    }
}