# Architecture - A technical guide to the project
| **Crate**       | **Description**                                                                                                                                   |
|-----------------|---------------------------------------------------------------------------------------------------------------------------------------------------|
| objektdb_core   | Contains all those functions that need to be executed at runtime                                                                                  |
| objektdb_macros | This is a procedural macro crate and contains all those macros that, when applied to your own structs, allow your code to interact with databases |

## objectdb_core
The purpose of this crate is to implement a form of storage engine that allows direct interaction with the binary files within which the data is contained. Let's take a look at the structures and logic by which the binaries are put written(you can review the same structures within the code in the comments and also in the documentation with `cargo test --open`):

A folder is created for each database with the name of the database and a file always with the same name with the extension `.db`:

```json
{
    magic_number,
    version,
    num_of_tables,
    flags
}
```

| **Field**     | **Purpose**                                                                                          | **Dimension** |
|---------------|------------------------------------------------------------------------------------------------------|---------------|
| magic_number  | Identifies the file as a valid objektDB database                                                     | 4 bytes       |
| version       | Database format version                                                                              | 1 byte        |
| num_of_tables | Identifies the number of tables contained within the database                                        | 1 byte        |
| flags         | For future use in case special features were to be introduced: each bit would be mapped to a feature | 4 bytes       |


Instead, a single table is represented by a file with a `.tbl` extension with the following format:

```json
HEADER{
    struct_name,
    offset_header,
    offset_index,
    offset_bucket,
    last_OID

    References{
        references_num,
    	struct_name1,
    	struct_name1
    }
    StructStructure{
        length_fields
        {
            length_field,
            field1,
            is_fk,
            length_type,
            type
        }
        {
            length_field,
            field2,
            is_fk,
            length_type,
            type
        }
        {
            length_method,
            method_name1
        }
        {
            length_method,
            method_name2
        }
    }
}
INDEX{
	
}
BUCKET{
    {
        OID
        Address(In Data section)
        Address_next_node(in caso di collisioni)
    }
}
DATA{
	Istance1{
		OID
		field1_value
		field2_value
	}
}

```

As you can see, the structure for the `.tbl` file gets complicated, defining several sections whose contents we are going to examine one by one below:

### Header

| **Field**      | **Purpose**                                                                                                                                                             | **Dimension**     |
|----------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------|
| struct_name    | It's the struct name, so even the database name                                                                                                                         | 64 bytes          |
| offset_header  | Where the header ends                                                                                                                                                   | 16 bytes          |
| offset_index   | The number of bytes from the beginning of the index(i.e., the address following the end of the header) to its end                                                       | 16 byte           |
| offset_bucket  | The number of bytes from the beginning of the bucket to its end                                                                                                         | 16 bytes          |
| last_OID       | It is the last object id assigned. It helps to assign another one faster                                                                                                | 64 bytes          |
| references_num | Number of references to external tables                                                                                                                                 | 8 bytes           |
| struct_name    | The generic name of a structure referenced in the table                                                                                                                 | 64 byte(per ref)  |
| length_fields  | The number of bytes from the beginning of the first field to the end of the last, where the methods begin. The end of the methods is where the header offset is instead | 16 byte           |
| length_field   | The number of bytes of the field name                                                                                                                                   | 8 byte            |
| field          | The field name                                                                                                                                                          | variable(max 255) |
| is_fk          | Specifies whether the field is a foreign key or not                                                                                                                     | 1 byte            |
| length_type    | The number of bytes of the type name                                                                                                                                    | 8 byte            |
| type           | Name of the type. It will be used for casting                                                                                                                           | variable(max 255) |
| length_method  | The number of bytes of the type name                                                                                                                                    | 8 byte            |
| method_name    | The name of the methods of the struct. They'll be used for logging purpose                                                                                              | variable(max 255) |
