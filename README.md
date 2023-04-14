# PtInvoiceParser

Starting on January 2022 all invoices in Portugal will have to be issued with a QR code that containing [most of the information](https://www.softseguro.pt/Files/ETCODEQR.pdf) of the invoice. 

This projects explores two different ways to parse portuguese invoices. One built with Rust that provides a simple GUI where invoices can be grabbed from the camera or from the screen and another built with Python that parses invoices from a folder, ideal for batch processing.

## Rust Invoice Parser
![Rust Invoice Parser](preview.gif)

The main motivation for this project was to learn Rust and to explore the possibility of building multi threaded GUI application with a glimpse of computer vision.
This application uses OpenCV with 2 threads to grab images from the screen or from the camera, parse the QR code and then displaying the contents of it on the screen.
In addition to that, it provides a way determine the subset of invoices that best matches a specific values, this can be usefull for Bank reconciliations in the accounting department.

### Building
To build the project it might be needed to install OpenCV libraries. Head over to [Rust installation guidelines](https://github.com/twistedfall/opencv-rust/blob/master/INSTALL.md)

## Python Invoice Parser
TODO
