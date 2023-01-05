#[macro_use]
pub(crate) mod crate_macros {
    /// Macro to read an exact buffer
    macro_rules! read_exact_buff {
        ($bufid:ident, $rdr:expr, $buflen:expr) => {{
            let mut $bufid = [0_u8; $buflen];
            let _ = $rdr.read_exact(&mut $bufid)?;
            $bufid
        }};
    }
}

#[macro_use]
pub mod pub_macros {

    /// Macro to create const for partition types.
    macro_rules! partition_types {
        (
            $(
                $(#[$docs:meta])*
                ($upcase:ident, $guid:expr, $os:expr)$(,)*
            )+
        ) => {
            $(
                $(#[$docs])*
                pub const $upcase: Type = Type {
                    guid: uuid::uuid!($guid),
                    os: $os,
                };
            )+

            impl Type {
                /// Lookup a partition type by uuid to populate the operating system
                /// field, if the type is previously known.
                pub fn from_uuid(u: &uuid::Uuid) -> Self {
                    match u.hyphenated().encode_upper(&mut Uuid::encode_buffer()).as_ref() {
                        $(
                            $guid => $upcase,
                        )+
                        _ => Self {
                            guid: *u,
                            os: OperatingSystem::None,
                        },
                    }
                }

                /// Attempt to derive the partition type from the upper-case formatted
                /// partition name
                ///
                /// ## Example
                /// ```rust
                /// assert_eq!(
                ///     gpt::partition_types::Type::from_name("LINUX_FS"),
                ///     Ok(gpt::partition_types::LINUX_FS)
                /// );
                /// ```
                pub fn from_name(s: &str) -> Result<Self, &'static str> {
                    match s {
                        $(
                            stringify!($upcase) => Ok($upcase),
                        )+
                        _ => Err("Invalid or unknown Partition name"),
                    }
                }
            }
        }
    }
}
