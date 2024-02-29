# glutin_simple1

A subtle flickering issue was happening with [iced](https://github.com/iced-rs/iced) but then i've noticed it with [egui](https://github.com/emilk/egui) as well, and tracked it down to [glutin](https://github.com/rust-windowing/glutin) but didn't try to go deeper than that, as it's already way past my current understanding of how things work.  

This flickering issue is happening only when all of the below are true at the same time:  

<details>

<summary>
NVidia G-SYNC is enabled for windowed mode
</summary>

[![gsync link](https://github.com/correabuscar/glutin_simple1/blob/main/nvidiagsync.jpg)](https://github.com/correabuscar/glutin_simple1/blob/main/nvidiagsync.jpg)

</details>



<details>

<summary>
displayed on an LG 24UD58-B monitor (manufactured: January 2018) with FreeSync->Extended (but not with Basic, or Off) setting in its OSD menu(The on-screen display menu).
</summary>

[![OSD1of3](https://github.com/correabuscar/glutin_simple1/blob/main/1IMG_20230503_101327-1.jpg)](https://github.com/correabuscar/glutin_simple1/blob/main/1IMG_20230503_101327-1.jpg)

[![OSD2of3](https://github.com/correabuscar/glutin_simple1/blob/main/2IMG_20230503_101338-1.jpg)](https://github.com/correabuscar/glutin_simple1/blob/main/2IMG_20230503_101338-1.jpg)

[![OSD3of3](https://github.com/correabuscar/glutin_simple1/blob/main/3IMG_20230503_101350-1.jpg)](https://github.com/correabuscar/glutin_simple1/blob/main/3IMG_20230503_101350-1.jpg)

</details>

<details>

<summary>
`context.swap_buffers()` is called either directly(as it is in this here project) or indirectly (iced, egui)
</summary>

https://github.com/correabuscar/glutin_simple1/blob/00bbecfdc7b6a511da0656ce67fc1b192fe2d450/src/main.rs#L44

</details>

The following video attempts to show the SUBTLE flickering that happens only when resizing of window is going on(as this is when `swap_buffers()` is called, but not when moving the window), but this flickering is much more visible when viewed on mobile screens and is almost imperceptible(except in real life) otherwise:


[![click to open video in new tab then choose Raw](https://raw.githubusercontent.com/correabuscar/glutin_simple1/main/thisVID_20230503_100650.mp4)](https://github.com/correabuscar/glutin_simple1/blob/main/thisVID_20230503_100650.mp4)


Workaround
==========

Use FreeSync->Basic (or Off)

WARNING
=======

While running it uses 100% CPU (in top, aka 1 full core), so don't let it run unsupervised. Unsure if it needs to run this fast, else I could've put a delay in the loop. I'm unable to (re)test at the time of writing this line.  
