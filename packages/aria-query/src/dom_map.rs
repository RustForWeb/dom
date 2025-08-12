use std::{hash::RandomState, sync::LazyLock};

use ordered_hash_map::OrderedHashMap;

use crate::types::{AriaDomDefinition, AriaQueryHtmlElement};

pub static DOM: LazyLock<OrderedHashMap<AriaQueryHtmlElement, AriaDomDefinition, RandomState>> =
    LazyLock::new(|| {
        OrderedHashMap::from_iter([
            (
                AriaQueryHtmlElement::A,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Abbr,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Acronym,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Address,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Applet,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Area,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Article,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Aside,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Audio,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::B,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Base,
                AriaDomDefinition { reserved: true },
            ),
            (
                AriaQueryHtmlElement::Bdi,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Bdo,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Big,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Blink,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Blockquote,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Body,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Br,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Button,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Canvas,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Caption,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Center,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Cite,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Code,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Col,
                AriaDomDefinition { reserved: true },
            ),
            (
                AriaQueryHtmlElement::Colgroup,
                AriaDomDefinition { reserved: true },
            ),
            (
                AriaQueryHtmlElement::Content,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Data,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Datalist,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Dd,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Del,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Details,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Dfn,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Dialog,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Dir,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Div,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Dl,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Dt,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Em,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Embed,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Fieldset,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Figcaption,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Figure,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Font,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Footer,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Form,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Frame,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Frameset,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::H1,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::H2,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::H3,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::H4,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::H5,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::H6,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Head,
                AriaDomDefinition { reserved: true },
            ),
            (
                AriaQueryHtmlElement::Header,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Hgroup,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Hr,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Html,
                AriaDomDefinition { reserved: true },
            ),
            (
                AriaQueryHtmlElement::I,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Iframe,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Img,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Input,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Ins,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Kbd,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Keygen,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Label,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Legend,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Li,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Link,
                AriaDomDefinition { reserved: true },
            ),
            (
                AriaQueryHtmlElement::Main,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Map,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Mark,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Marquee,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Menu,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Menuitem,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Meta,
                AriaDomDefinition { reserved: true },
            ),
            (
                AriaQueryHtmlElement::Meter,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Nav,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Noembed,
                AriaDomDefinition { reserved: true },
            ),
            (
                AriaQueryHtmlElement::Noscript,
                AriaDomDefinition { reserved: true },
            ),
            (
                AriaQueryHtmlElement::Object,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Ol,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Optgroup,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Option,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Output,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::P,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Param,
                AriaDomDefinition { reserved: true },
            ),
            (
                AriaQueryHtmlElement::Picture,
                AriaDomDefinition { reserved: true },
            ),
            (
                AriaQueryHtmlElement::Pre,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Progress,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Q,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Rp,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Rt,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Rtc,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Ruby,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::S,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Samp,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Script,
                AriaDomDefinition { reserved: true },
            ),
            (
                AriaQueryHtmlElement::Section,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Select,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Small,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Source,
                AriaDomDefinition { reserved: true },
            ),
            (
                AriaQueryHtmlElement::Spacer,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Span,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Strike,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Strong,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Style,
                AriaDomDefinition { reserved: true },
            ),
            (
                AriaQueryHtmlElement::Sub,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Summary,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Sup,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Table,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Tbody,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Td,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Textarea,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Tfoot,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Th,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Thead,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Time,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Title,
                AriaDomDefinition { reserved: true },
            ),
            (
                AriaQueryHtmlElement::Tr,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Track,
                AriaDomDefinition { reserved: true },
            ),
            (
                AriaQueryHtmlElement::Tt,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::U,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Ul,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Var,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Video,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Wbr,
                AriaDomDefinition { reserved: false },
            ),
            (
                AriaQueryHtmlElement::Xmp,
                AriaDomDefinition { reserved: false },
            ),
        ])
    });
