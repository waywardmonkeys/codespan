//! Diagnostic reporting support for the codespan crate

use codespan::ByteSpan;

use Severity;

/// A style for the label
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum LabelStyle {
    /// The main focus of the diagnostic
    Primary,
    /// Supporting labels that may help to isolate the cause of the diagnostic
    Secondary,
}

/// A label describing an underlined region of code associated with a diagnostic
#[derive(Clone, Debug)]
pub struct Label {
    /// The span we are going to include in the final snippet.
    pub span: ByteSpan,
    /// A message to provide some additional information for the underlined code.
    pub message: Option<String>,
    /// The style to use for the label.
    pub style: LabelStyle,
}

impl Label {
    pub fn new(span: ByteSpan, style: LabelStyle) -> Label {
        Label {
            span,
            message: None,
            style,
        }
    }

    pub fn new_primary(span: ByteSpan) -> Label {
        Label::new(span, LabelStyle::Primary)
    }

    pub fn new_secondary(span: ByteSpan) -> Label {
        Label::new(span, LabelStyle::Primary)
    }

    pub fn with_message<S: Into<String>>(mut self, message: S) -> Label {
        self.message = Some(message.into());
        self
    }
}

/// Represents a diagnostic message and associated child messages.
#[derive(Clone, Debug)]
pub struct Diagnostic {
    /// The overall severity of the diagnostic
    pub severity: Severity,
    /// The main message associated with this diagnostic
    pub message: String,
    /// The labelled spans marking the regions of code that cause this
    /// diagnostic to be raised
    pub labels: Vec<Label>,
}

impl Diagnostic {
    pub fn new<S: Into<String>>(severity: Severity, message: S) -> Diagnostic {
        Diagnostic {
            severity,
            message: message.into(),
            labels: Vec::new(),
        }
    }

    pub fn new_bug<S: Into<String>>(message: S) -> Diagnostic {
        Diagnostic::new(Severity::Bug, message)
    }

    pub fn new_error<S: Into<String>>(message: S) -> Diagnostic {
        Diagnostic::new(Severity::Error, message)
    }

    pub fn new_warning<S: Into<String>>(message: S) -> Diagnostic {
        Diagnostic::new(Severity::Warning, message)
    }

    pub fn new_note<S: Into<String>>(message: S) -> Diagnostic {
        Diagnostic::new(Severity::Note, message)
    }

    pub fn new_help<S: Into<String>>(message: S) -> Diagnostic {
        Diagnostic::new(Severity::Help, message)
    }

    pub fn with_label(mut self, label: Label) -> Diagnostic {
        self.labels.push(label);
        self
    }

    pub fn with_labels<Labels: IntoIterator<Item = Label>>(mut self, labels: Labels) -> Diagnostic {
        self.labels.extend(labels);
        self
    }
}
