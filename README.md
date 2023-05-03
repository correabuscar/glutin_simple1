# glutin_simple1

A subtle flickering issue was happening with [iced](https://github.com/iced-rs/iced) but then i've noticed it with [egui](https://github.com/emilk/egui) as well, and tracked it down to [glutin](https://github.com/rust-windowing/glutin) but didn't try to go deeper than that, as it's already way past my current understanding of how things work.  

This flickering issue is happening only when all of the below are true at the same time:  

<details>

<summary>
NVidia G-SYNC is enabled for windowed mode
</summary>

[![TODO: fix link here](https://github.com/correabuscar/glutin_simple1/blob/main/nvidiagsync.jpg)](https://github.com/correabuscar/glutin_simple1/blob/main/nvidiagsync.jpg)

</details>



<details>

<summary>
displayed on an LG 24UD58-B monitor (manufactured: January 2018) with FreeSync->Extended (but not with Basic, or Off) setting.
</summary>

</details>

<details>

<summary>
`context.swap_buffers()` is called either directly(as it is in this here project) or indirectly (iced, egui)
</summary>
</details>

The following video attempts to show the SUBTLE flickering that happens only when resizing of window is going on(as this is when `swap_buffers()` is called, but not when moving the window), but this flickering is much more visible when viewed on mobile screens and is almost imperceptible(except in real life) otherwise:


[![TODO: add link here](https://github.com/correabuscar/glutin_simple1/blob/main/thisVID_20230503_100650.mp4)](https://github.com/correabuscar/glutin_simple1/blob/main/thisVID_20230503_100650.mp4)

