from PIL import Image
from itertools import product


def convert_to_overlay(canvas_dim, images):
    cw, ch = canvas_dim
    canvas_dim = cw * 3, ch * 3

    overlay_img = Image.new(mode="RGBA", size=canvas_dim, color=(0, 0, 0, 0))

    def adjusted(location, length):
        return map(lambda a: (a, (location + a) * 3 + 1), range(length))

    for place_img, location in images:
        lx, ly = location
        for (x, x2), (y, y2) in product(
            adjusted(lx, place_img.width),
            adjusted(ly, place_img.height),
        ):
            overlay_img.putpixel((x2, y2), place_img.getpixel((x, y)))

    return overlay_img


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("out")

    parser.add_argument(
        "--template", metavar=("IMG", "X", "Y"), nargs=3, action="append"
    )

    args = parser.parse_args()

    images = [(Image.open(path), (int(x), int(y))) for path, x, y in args.template]

    canvas_dim = 2000, 2000
    convert_to_overlay(canvas_dim, images).save(args.out)
