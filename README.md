# Rusty imager
Library that helps you manipulate images in rust

## Supported formats
* BMP
* PNG (planned)
* JPG (planned)

## Available filters
* Gaussian blur
* Color inversion
* Box blur
* Brightness
* Contrast
* Grayscale
* Threshold
* Oil painting
* Edge detection
* Emboss (not yet implemented)

## Instalation
Add this library to your project with cargo
```sh
cargo add --git "https://github.com/J1rk0s/rusty_imager.git"
```

## Basic usage
### Applying filters to an image
```rs
use rusty_imager::Image;
use rusty_imager::filters::<FILTER NAME>;

let img = Image::from_file("<path>").expect("Failed to open the image");
img.apply_filter(<FILTER NAME>);

img.save("<name>.bmp");
```

### Getting a single pixel
```rs
use rusty_imager::{Image, formats::ImageFormat};

let img = Image::from_file("<path>").expect("Failed to open the image");

match img.get_pixel(x, y) {
    Some(pixel) => {
        // Do something with the pixel
    },

    None => {
        // Do something
    }
}

img.save("<name>.bmp");
```

### Defining a custom filter
```rs
use rusty_imager::{filters::ImageFilter, formats::ImageFormat};

pub struct Test {
    // Fields
}

impl Test {
    pub fn new(/* Args */) -> Self {
        Self {
            // Fields
        }
    }
}

impl ImageFilter for Test {
    fn apply(&self, img: &mut Box<dyn ImageFormat>) {
        // Do something
    }
}
```