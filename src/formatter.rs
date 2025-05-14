use std::io;

pub trait Formatter {
    #[inline]
    fn write_null<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"#")
    }

    #[inline]
    fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let s = if value {
            b"%true" as &[u8]
        } else {
            b"%false" as &[u8]
        };
        writer.write_all(s)
    }

    #[inline]
    fn write_i8<W>(&mut self, writer: &mut W, value: i8) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        write!(writer, "{}", value)
    }

    #[inline]
    fn write_i16<W>(&mut self, writer: &mut W, value: i16) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        write!(writer, "{}", value)
    }

    #[inline]
    fn write_i32<W>(&mut self, writer: &mut W, value: i32) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        write!(writer, "{}", value)
    }

    #[inline]
    fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        write!(writer, "{}", value)
    }

    #[inline]
    fn write_i128<W>(&mut self, writer: &mut W, value: i128) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        write!(writer, "{}", value)
    }

    #[inline]
    fn write_u8<W>(&mut self, writer: &mut W, value: u8) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        write!(writer, "{}u", value)
    }

    #[inline]
    fn write_u16<W>(&mut self, writer: &mut W, value: u16) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        write!(writer, "{}u", value)
    }

    #[inline]
    fn write_u32<W>(&mut self, writer: &mut W, value: u32) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        write!(writer, "{}u", value)
    }

    #[inline]
    fn write_u64<W>(&mut self, writer: &mut W, value: u64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {        
        write!(writer, "{}u", value)
    }

    #[inline]
    fn write_u128<W>(&mut self, writer: &mut W, value: u128) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        write!(writer, "{}u", value)
    }

    #[inline]
    fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        write!(writer, "{}", value)
    }

    #[inline]
    fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        write!(writer, "{}", value)
    }

    #[inline]
    fn write_number_str<W>(&mut self, writer: &mut W, value: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(value.as_bytes())
    }

    #[inline]
    fn begin_string<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")
    }

    #[inline]
    fn end_string<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")
    }

    #[inline]
    fn write_string_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(fragment.as_bytes())
    }

    // #[inline]
    // fn write_char_escape<W>(&mut self, writer: &mut W, char_escape: CharEscape) -> io::Result<()>
    // where
    //     W: ?Sized + io::Write,
    // {
    //     use self::CharEscape::*;

    //     let s = match char_escape {
    //         Quote => b"\\\"",
    //         ReverseSolidus => b"\\\\",
    //         Solidus => b"\\/",
    //         Backspace => b"\\b",
    //         FormFeed => b"\\f",
    //         LineFeed => b"\\n",
    //         CarriageReturn => b"\\r",
    //         Tab => b"\\t",
    //         AsciiControl(byte) => {
    //             static HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";
    //             let bytes = &[
    //                 b'\\',
    //                 b'u',
    //                 b'0',
    //                 b'0',
    //                 HEX_DIGITS[(byte >> 4) as usize],
    //                 HEX_DIGITS[(byte & 0xF) as usize],
    //             ];
    //             return writer.write_all(bytes);
    //         }
    //     };

    //     writer.write_all(s)
    // }

    fn write_byte_list<W>(&mut self, writer: &mut W, value: &[u8]) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        (self.begin_list(writer))?;
        let mut first = true;
        for byte in value {
            (self.begin_list_value(writer, first))?;
            (self.write_u8(writer, *byte))?;
            (self.end_list_value(writer))?;
            first = false;
        }
        self.end_list(writer)
    }

    #[inline]
    fn begin_list<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"[")
    }

    #[inline]
    fn end_list<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"]")
    }

    #[inline]
    fn begin_list_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if first {
            Ok(())
        } else {
            writer.write_all(b";")
        }
    }

    #[inline]
    fn end_list_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        Ok(())
    }

    #[inline]
    fn begin_map<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"{")
    }

    #[inline]
    fn end_map<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"}")
    }

    #[inline]
    fn begin_map_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if first {
            Ok(())
        } else {
            writer.write_all(b";")
        }
    }

    #[inline]
    fn end_map_key<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        Ok(())
    }

    #[inline]
    fn begin_map_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"=")
    }

    #[inline]
    fn end_map_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        Ok(())
    }

    #[inline]
    fn write_raw_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(fragment.as_bytes())
    }

    #[inline]
    fn begin_attributes<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"<")
    }

    #[inline]
    fn end_attributes<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b">")
    }
}

#[derive(Clone, Debug)]
pub struct TextFormatter;

impl Formatter for TextFormatter {}

#[derive(Clone, Debug)]
pub struct PrettyFormatter<'a> {
    current_indent: usize,
    has_value: bool,
    indent: &'a [u8],
}

impl<'a> PrettyFormatter<'a> {
    pub fn new() -> Self {
        PrettyFormatter::with_indent(b"  ")
    }

    pub fn with_indent(indent: &'a [u8]) -> Self {
        PrettyFormatter {
            current_indent: 0,
            has_value: false,
            indent,
        }
    }
}

impl<'a> Default for PrettyFormatter<'a> {
    fn default() -> Self {
        PrettyFormatter::new()
    }
}

impl<'a> Formatter for PrettyFormatter<'a> {
    #[inline]
    fn begin_list<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;
        writer.write_all(b"[")
    }

    #[inline]
    fn end_list<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent -= 1;

        if self.has_value {
            (writer.write_all(b"\n"))?;
            (indent(writer, self.current_indent, self.indent))?;
        }

        writer.write_all(b"]")
    }

    #[inline]
    fn begin_list_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        (writer.write_all(if first { b"\n" } else { b";\n" }))?;
        indent(writer, self.current_indent, self.indent)
    }

    #[inline]
    fn end_list_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.has_value = true;
        Ok(())
    }

    #[inline]
    fn begin_map<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;
        writer.write_all(b"{")
    }

    #[inline]
    fn end_map<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent -= 1;

        if self.has_value {
            (writer.write_all(b"\n"))?;
            (indent(writer, self.current_indent, self.indent))?;
        }

        writer.write_all(b"}")
    }

    #[inline]
    fn begin_map_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        (writer.write_all(if first { b"\n" } else { b";\n" }))?;
        indent(writer, self.current_indent, self.indent)
    }

    #[inline]
    fn begin_map_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b" = ")
    }

    #[inline]
    fn end_map_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.has_value = true;
        Ok(())
    }

        #[inline]
    fn begin_attributes<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;
        writer.write_all(b"<")
    }

    #[inline]
    fn end_attributes<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent -= 1;

        if self.has_value {
            (writer.write_all(b"\n"))?;
            (indent(writer, self.current_indent, self.indent))?;
        }

        writer.write_all(b"> ")
    }
}


fn indent<W>(w: &mut W, n: usize, s: &[u8]) -> io::Result<()>
where
    W: ?Sized + io::Write,
{
    for _ in 0..n {
        (w.write_all(s))?;
    }

    Ok(())
}


// pub struct BinaryFormatter;

// impl Formatter for BinaryFormatter {
//     #[inline]
//     fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> io::Result<()>
//     where
//         W: ?Sized + io::Write,
//     {
//         let s = if value {
//             b"\x04" as &[u8]
//         } else {
//             b"\x05" as &[u8]
//         };
//         writer.write_all(s)
//     }
// }

mod to;
pub use self::to::{to_writer, to_vec, to_string};
