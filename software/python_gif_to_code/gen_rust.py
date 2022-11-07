import gif2numpy
import cv2
import pandas 
import numpy
np_frames, extensions, image_specifications = gif2numpy.convert("rainbow.gif")
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
print(f"let frames = [")
for i, frame in enumerate(np_frames):
    print("[")
    for led in led_locs:
        led_x = led[3]
        led_y = led[4]
        pixel_x = int(numpy.interp(led_x, (led_min_x, led_max_x), (0, gif_max_x-1)))
        pixel_y = int(numpy.interp(led_y, (led_min_y, led_max_y), (0, gif_max_y-1)))
        led_red = frame[pixel_y][pixel_x][0]
        led_green = frame[pixel_y][pixel_x][1]
        led_blue = frame[pixel_y][pixel_x][2]
        print(f"[{led_red}, {led_green}, {led_blue}], ")
    print("],")
print("];")
