from rusty_manga_image_translator import Session, PyPreprocessorOptions, PyDefaultOptions, PyImage
#import numpy as np
#from PIL import Image

# det = Session(["cuda", "directml", "tensorrt", "coreml"])
ses = Session(None)

# det = ses.convnext_detector()
det = ses.default_detector()

o1 = PyPreprocessorOptions(False, False, False, False)
o2 = PyDefaultOptions(2048, 2.3, 0.5, 0.7)


if (not det.loaded()):
    det.load()


# img = PyImage.from_numpy(np.array(Image.open("./imgs/232264684-5a7bcf8e-707b-4925-86b0-4212382f1680.png")))
img = PyImage("./imgs/232264684-5a7bcf8e-707b-4925-86b0-4212382f1680.png")
import time

start_time = time.time()
areas, mask = det.detect(img, o1, o2)
end_time = time.time()
execution_time = end_time - start_time
print(f"Execution time of detect: {execution_time} seconds")

det.unload()
