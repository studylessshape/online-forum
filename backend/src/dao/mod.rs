use crate::utils::into_mem;

pub mod connect;
pub mod dao_error;
pub mod email;
pub mod post;
pub mod section;
pub mod user;
pub mod comment;

/// This trait should implement the function to back this statements and the params placed in statements.
///
/// These provide to mysql crate. It's can be found on [mysql doc](https://docs.rs/mysql/latest/mysql/#named-parameters)
pub trait ConditionFor<T> {
    fn get_condition(&self) -> (String, mysql::Params);
}

/// Get `Value` to `T`, if err, the ownership of `row` will be steel but not panic
pub fn get_value<T>(value: &mysql::Value, row: &mysql::Row) -> Result<T, mysql::FromRowError>
where
    T: mysql::prelude::FromValue,
{
    match T::from_value_opt(value.into()) {
        Ok(val) => Ok(val),
        Err(_) => Err(mysql::FromRowError(into_mem(row))),
    }
}
