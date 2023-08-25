/*
Bicycle is a database, used for things databases are used for.

Copyright (C) 2023  sean watters

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use prost_types::{field_descriptor_proto::Type, DescriptorProto, FieldDescriptorProto};

#[derive(Debug)]
pub struct Property {
    pub _type: String,
    pub name: String,
    pub number: i32,
}

#[derive(Debug)]
pub struct Model {
    pub name: String,
    pub properties: Vec<Property>,
    pub nested_models: Vec<Model>,
}

pub fn construct_model(
    message: &DescriptorProto,
    should_check_pk: bool,
) -> Result<Model, &'static str> {
    let mut has_valid_pk = false;
    let mut properties: Vec<Property> = vec![];

    for field in message.field.iter() {
        properties.push(Property {
            _type: get_usable_type(&field, &message),
            name: field.name().to_string(),
            number: field.number(),
        });

        if field.name() == "pk" {
            if field.number() == 1 {
                match field.r#type() {
                    Type::String => {
                        has_valid_pk = true;
                    }
                    _ => eprintln!("missing 'string pk = 1;'"),
                }
            }
        }
    }

    if should_check_pk && !has_valid_pk {
        return Err("model does not include `string pk = 1;`");
    }

    let mut nested_models: Vec<Model> = vec![];

    for nested_message in message.nested_type.iter() {
        if nested_message.name().ends_with("Entry") {
            continue;
        }

        let nested_model = construct_model(&nested_message, false)?;
        nested_models.push(nested_model);
    }

    Ok(Model {
        name: message.name().to_string(),
        properties,
        nested_models,
    })
}

pub fn get_complex_type(field: &FieldDescriptorProto, message: &DescriptorProto) -> String {
    if let Some(type_name) = field.type_name().split('.').last() {
        for nested_type in message.nested_type.iter() {
            if nested_type.name() == type_name {
                // check for map<,> type
                if type_name.ends_with("Entry") {
                    let mut key_type = "".to_string();
                    let mut val_type = "".to_string();

                    for field in nested_type.field.iter() {
                        if field.name() == "key" {
                            key_type = get_usable_type(&field, &message);
                        } else if field.name() == "value" {
                            val_type = get_usable_type(&field, &message);
                        }
                    }

                    return format!("map<{}, {}>", key_type, val_type);
                } else {
                    return type_name.to_string();
                }
            }
        }

        return "".to_string();
    }

    "".to_string()
}

pub fn get_usable_type(field: &FieldDescriptorProto, message: &DescriptorProto) -> String {
    match field.r#type() {
        Type::Double => "double".to_string(),
        Type::Float => "float".to_string(),
        Type::Int32 => "int32".to_string(),
        Type::Int64 => "int64".to_string(),
        Type::Uint32 => "uint32".to_string(),
        Type::Uint64 => "uint64".to_string(),
        Type::Sint32 => "sint32".to_string(),
        Type::Sint64 => "sint64".to_string(),
        Type::Fixed32 => "fixed32".to_string(),
        Type::Fixed64 => "fixed64".to_string(),
        Type::Sfixed32 => "sfixed32".to_string(),
        Type::Sfixed64 => "sfixed64".to_string(),
        Type::Bool => "bool".to_string(),
        Type::String => "string".to_string(),
        Type::Bytes => "bytes".to_string(),
        Type::Message => get_complex_type(field, message),

        // !! handle explicitly
        Type::Enum => "ENUM".to_string(),
        Type::Group => "GROUP IS NOT SUPPORTED".to_string(),
    }
}
