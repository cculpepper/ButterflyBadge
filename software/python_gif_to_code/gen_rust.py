import gif2numpy
import cv2
import pandas 
import numpy
import pysvg.structure
import pysvg.builders
import pysvg.text


np_frames, extensions, image_specifications = gif2numpy.convert("rainbow.gif")
#  np_frames, extensions, image_specifications = gif2numpy.convert("bear.gif")
#  np_frames, extensions, image_specifications = gif2numpy.convert("scrolling_rainbow.gif")
led_locs = pandas.read_csv("LED_Positions.csv").to_numpy()
#print(led_locs)


#print(image_specifications)
gif_max_x = image_specifications["Image Size"][0]
gif_max_y = image_specifications["Image Size"][1]

led_max_y = -99999
led_max_x = -99999
led_min_y = 99999
led_min_x = 99999
for led in led_locs:
    if led[3] < led_min_x:
        led_min_x = led[3]
    if led[4] < led_min_y:
        led_min_y = led[4]
    if led[3] > led_max_x:
        led_max_x = led[3]
    if led[4] > led_max_y:
        led_max_y = led[4]
#print(f"LED Max: {led_max_x}, {led_max_y} , LED Min: {led_min_x}, {led_min_y}")
print(f"pub static frames : [[[u8;3];512];{len(np_frames)}]= [")
for i, frame in enumerate(np_frames):
    svg_document = pysvg.structure.Svg()
    shape_builder = pysvg.builders.ShapeBuilder()
    print("[")
    for led in led_locs:
        led_x = led[3]
        led_y = led[4]
        pixel_x = int(numpy.interp(led_x, (led_min_x, led_max_x), (0, gif_max_x-1)))
        pixel_y = int(numpy.interp(led_y, (led_min_y, led_max_y), (0, gif_max_y-1)))
        led_red = frame[pixel_y][pixel_x][2]
        led_red = frame[pixel_y][pixel_x][2]
        led_green = frame[pixel_y][pixel_x][1]
        led_blue = frame[pixel_y][pixel_x][0]
        if False:#led_y < 100 or led_y >250:
            led_red = 0;
            led_blue = 0;
            led_green = 0;
        print(f"[{led_red}, {led_green}, {led_blue}],  // Desig {led[2]}  {led_x}, {led_y}")
        svg_document.addElement(shape_builder.createCircle(pixel_x, pixel_y, 5, fill=f"rgb({led_red}, {led_green}, {led_blue})"))
    print("],")
    #  svg_document.save(f"gif/frame{i:02}.svg")
print("];")
