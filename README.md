# RIMP - Rust Image Processor

A CLI tool for processing images

## Run

```sh
#############
# Resize
#############

# Resize ./image.png to ./image-out.png with width and height of 100 using default Lanczos3 filter
rimp resize --width 100 --height 100 ./image.png

# Resize ./image.png with width and height 100 less than original
rimp resize --width -100 --height -100 ./image.png

# Resize ./image.png with width and height half of original
rimp resize --percent 50 ./image.png

# Resize ./image.png using Nearest Neighbor filter
rimp resize --width 100 --height 100 --filter nearest ./image.png

# Resize ./image.png and write file to ./image2.png
rimp resize --width 100 --height 100 --output ./image2.png ./image.png

# Resize ./image.png and convert to JPEG
rimp resize --width 100 --height 100 --output ./image-out.jpg ./image.png
```
