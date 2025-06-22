from rusty_manga_mange_translator import Session, PyPreprocessorOptions, PyDefaultOptions, PyImage

# det = Session(["cuda", "directml", "tensorrt", "coreml"])
ses = Session(None)

# det = ses.convnext_detector()
det = ses.default_detector()

o1 = PyPreprocessorOptions(False, False, False, False)
o2 = PyDefaultOptions(2048, 2.3, 0.5, 0.7)


if (not det.loaded()):
    det.load()


# img = PyImage.from_numpy(array)
img = PyImage("./test.png")
det.detect(img, o1, o2)

det.unload()
