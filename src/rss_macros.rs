#[macro_export]
macro_rules! macro_generate_rss_custom {
    ($writer:expr, $options:expr) => {{
        use quick_xml::events::{
            BytesDecl, BytesEnd, BytesStart, BytesText, Event,
        };

        let mut writer = $writer;

        writer.write_event(Event::Decl(BytesDecl::new(
            "1.0",
            Some("utf-8"),
            None,
        )))?;

        let mut rss_start = BytesStart::new("rss");
        rss_start.push_attribute(("version", "2.0"));
        rss_start.push_attribute((
            "xmlns:atom",
            "http://www.w3.org/2005/Atom",
        ));
        writer.write_event(Event::Start(rss_start))?;

        writer.write_event(Event::Start(BytesStart::new("channel")))?;

        macro_write_element!(writer, "title", &$options.title)?;
        macro_write_element!(writer, "link", &$options.link)?;
        macro_write_element!(
            writer,
            "description",
            &$options.description
        )?;
        macro_write_element!(writer, "language", &$options.language)?;
        macro_write_element!(writer, "pubDate", &$options.pub_date)?;
        macro_write_element!(
            writer,
            "lastBuildDate",
            &$options.last_build_date
        )?;
        macro_write_element!(writer, "docs", &$options.docs)?;
        macro_write_element!(writer, "generator", &$options.generator)?;
        macro_write_element!(
            writer,
            "managingEditor",
            &$options.managing_editor
        )?;
        macro_write_element!(writer, "webMaster", &$options.webmaster)?;
        macro_write_element!(writer, "category", &$options.category)?;
        macro_write_element!(writer, "ttl", &$options.ttl)?;

        // Write image element
        if !$options.image_url.is_empty() {
            writer
                .write_event(Event::Start(BytesStart::new("image")))?;
            macro_write_element!(writer, "url", &$options.image_url)?;
            macro_write_element!(writer, "title", &$options.title)?;
            macro_write_element!(writer, "link", &$options.link)?;
            writer.write_event(Event::End(BytesEnd::new("image")))?;
        }

        // Write atom:link
        if !$options.atom_link.is_empty() {
            let mut atom_link_start = BytesStart::new("atom:link");
            atom_link_start
                .push_attribute(("href", $options.atom_link.as_str()));
            atom_link_start.push_attribute(("rel", "self"));
            atom_link_start
                .push_attribute(("type", "application/rss+xml"));
            writer.write_event(Event::Empty(atom_link_start))?;
        }

        // Write item
        for item in $options.items {
            writer.write_event(Event::Start(BytesStart::new("item")))?;
            macro_write_element!(writer, "title", item.title.as_str())?;
            macro_write_element!(writer, "link", item.link.as_str())?;
            macro_write_element!(
                writer,
                "description",
                &$options.description
            )?;
            macro_write_element!(writer, "author", item.author.as_str())?;
            macro_write_element!(writer, "guid", item.guid.as_str())?;
            macro_write_element!(writer, "pubDate", item.pub_date.as_str())?;
            writer.write_event(Event::End(BytesEnd::new("item")))?;
        }

        writer.write_event(Event::End(BytesEnd::new("channel")))?;
        writer.write_event(Event::End(BytesEnd::new("rss")))?;

        Ok(writer)
    }};
}
