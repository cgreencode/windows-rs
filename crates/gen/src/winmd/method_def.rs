use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct MethodDef(pub winmd::Row);

impl MethodDef {
    pub fn flags(self, reader: &TypeReader) -> winmd::MethodFlags {
        winmd::MethodFlags(reader.u32(self.0, 2))
    }

    pub fn parent(self, reader: &TypeReader) -> winmd::TypeDef {
        winmd::TypeDef(reader.upper_bound(
            self.0.file_index,
            winmd::TableIndex::TypeDef,
            6,
            self.0.index,
        ))
    }

    pub fn params(self, reader: &TypeReader) -> impl Iterator<Item = winmd::Param> {
        reader
            .list(self.0, winmd::TableIndex::Param, 5)
            .map(winmd::Param)
    }

    pub fn name(self, reader: &TypeReader) -> &str {
        reader.str(self.0, 3)
    }

    pub fn sig(self, reader: &TypeReader) -> winmd::Blob {
        reader.blob(self.0, 4)
    }

    pub fn category(self, reader: &TypeReader) -> winmd::MethodCategory {
        if self.flags(reader).special() {
            let name = self.name(reader);

            if name.starts_with("get") {
                winmd::MethodCategory::Get
            } else if name.starts_with("put") {
                winmd::MethodCategory::Set
            } else if name.starts_with("add") {
                winmd::MethodCategory::Add
            } else if name.starts_with("remove") {
                winmd::MethodCategory::Remove
            } else {
                // A delegate's 'Invoke' method is "special" but lacks a preamble.
                winmd::MethodCategory::Normal
            }
        } else {
            winmd::MethodCategory::Normal
        }
    }

    pub fn attributes(self, reader: &TypeReader) -> impl Iterator<Item = winmd::Attribute> {
        reader
            .equal_range(
                self.0.file_index,
                winmd::TableIndex::CustomAttribute,
                0,
                winmd::HasAttribute::MethodDef(self).encode(),
            )
            .map(winmd::Attribute)
    }

    pub fn find_attribute(
        self,
        reader: &TypeReader,
        name: (&str, &str),
    ) -> Option<winmd::Attribute> {
        self.attributes(reader)
            .find(|attribute| attribute.name(reader) == name)
    }
}
