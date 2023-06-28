# JMFM
JMFM (Java minecraft:filled_map's) is a library and console utility designed to convert images into Minecraft map art

# CLI Examples
How to use:
```shell
patsera@hp:~/Projects/jmfm~$ jmfm 
Blazingly fast conversion of images into Minecraft maps

Usage: jmfm [OPTIONS] <WORLD> <IMAGES>...

Arguments:
  <WORLD>      Root directory of the target world
  <IMAGES>...  Paths to image files

Options:
  -w, --width [<WIDTH>]              By default, the width is chosen automatically
  -h, --height [<HEIGHT>]            By default, the height is chosen automatically
  -i, --first-map-id <FIRST-MAP-ID>  ID of the first map in the order
patsera@hp:~/Projects/jmfm~$
```
PNG Image -> 16x9 map-art example:
```shell
patsera@hp:~/Projects/jmfm~$ jmfm -w=16 -h=9 ~/New\ World/ ~/image.png
[INFO  jmfm] Reading image files
[INFO  jmfm] The first map id will be 0
[INFO  jmfm] 1 image(s) -> 144 map(s)
[INFO  jmfm] AVG Speed: 114.0 maps/sec
patsera@hp:~/Projects/jmfm~$
```

You can also convert multiple images at once:
```shell
patsera@hp:~/Projects/jmfm$ jmfm -w=16 -h=9 ~/New\ World/ ~/image1.png ~/image2.png ~/image3.jpg
[INFO  jmfm] Reading image files
[INFO  jmfm] The first map id will be 144
[INFO  jmfm] 3 image(s) -> 432 map(s)
[INFO  jmfm] AVG Speed: 117.4 maps/sec
```
Or:
```shell
patsera@hp:~/Projects/jmfm~$ jmfm -w=16 -h=9 ~/New\ World/ ~/*
[INFO  jmfm] Reading image files
[INFO  jmfm] The first map id will be 576
[INFO  jmfm] 11 image(s) -> 1584 map(s)
[INFO  jmfm] AVG Speed: 106.0 maps/sec
patsera@hp:~/Projects/jmfm~$
```

### You must specify Minecraft map colors in your jmfm config
#### I personally use this:
<details><summary>default-config.toml</summary>

```toml
# https://minecraft.fandom.com/wiki/Data_version
data_version = 3337

# https://minecraft.fandom.com/wiki/Map_item_format
multipliers = [180, 220, 255, 135]
colors = [
    # 1.8
    [127, 178, 56],
    [247, 233, 163],
    [199, 199, 199],
    [255, 0, 0],
    [160, 160, 255],
    [167, 167, 167],
    [0, 124, 0],
    [255, 255, 255],
    [164, 168, 184],
    [151, 109, 77],
    [112, 112, 112],
    [64, 64, 255],
    [143, 119, 72],
    [255, 252, 245],
    [216, 127, 51],
    [178, 76, 216],
    [102, 153, 216],
    [229, 229, 51],
    [127, 204, 25],
    [242, 127, 165],
    [76, 76, 76],
    [153, 153, 153],
    [76, 127, 153],
    [127, 63, 178],
    [51, 76, 178],
    [102, 76, 51],
    [102, 127, 51],
    [153, 51, 51],
    [25, 25, 25],
    [250, 238, 77],
    [92, 219, 213],
    [74, 128, 255],
    [0, 217, 58],
    [129, 86, 49],
    [112, 2, 0],
    # 1.12
    [209, 177, 161],
    [159, 82, 36],
    [149, 87, 108],
    [112, 108, 138],
    [186, 133, 36],
    [103, 117, 53],
    [160, 77, 78],
    [57, 41, 35],
    [135, 107, 98],
    [87, 92, 92],
    [122, 73, 88],
    [76, 62, 92],
    [76, 50, 35],
    [76, 82, 42],
    [142, 60, 46],
    [37, 22, 16],
    # 1.16
    [189, 48, 49],
    [148, 63, 97],
    [92, 25, 29],
    [22, 126, 134],
    [58, 142, 140],
    [86, 44, 62],
    [20, 180, 133],
    # 1.17
    [100, 100, 100],
    [216, 175, 147],
    [127, 167, 150]
]
```

</details>
