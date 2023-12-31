use {super::Archive, crate::component::ComponentDetails, mime::Mime};

/// Component specific definitions in an [Archive].
pub trait ArchiveComponent {
    /// Options when loading the component.
    type Options;
    /// Loads the content of the component from disk into memory. Is also used
    /// to add additional pre-processing.
    fn load<A: Archive>(options: Self::Options) -> anyhow::Result<Self>
    where
        Self: Sized;
    /// The [Mime] type of the content.
    fn mime_type() -> Mime;
    /// Returns a reference to the unique [ComponentDetails] of this component.
    ///
    /// When implementing this trait, these details should be generated by your
    /// [load](Self::load) method and stored in the component.
    fn details(&self) -> &ComponentDetails;
}
