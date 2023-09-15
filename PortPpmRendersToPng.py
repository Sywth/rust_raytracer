import os, pathlib
import PIL as pil
from PIL import Image


def save_as_png(source_path : pathlib.Path, save_to_path : pathlib.Path):
    with open(source_path, 'r') as f:
        f.readline()# Skip PPM header
        width,height = f.readline().strip().split(' ')
        width,height = int(width),int(height)
        f.readline()# Skip max color value line

        iamge_arr = []
        for y in range(height):
            row = []
            for x in range(width):
                r,g,b = f.readline().strip().split(' ')
                row.append([int(r),int(g),int(b)])
            iamge_arr.append(row)
        
        # Convert to PIL image
        image = Image.new('RGB', (width, height))
        for y in range(height):
            for x in range(width):
                image.putpixel((x,y), tuple(iamge_arr[y][x]))
        image.save(save_to_path)

files = os.listdir("render")
for i,file in enumerate(files):
    try:
        save_as_png(f"render/{file}", f"render_pngs/ppm_{i}.png")
    except:
        print(f"Failed to convert {file} to png")

            
    
