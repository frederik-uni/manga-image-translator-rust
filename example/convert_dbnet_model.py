torch.onnx.export(
    MODEL,
    batch,
    "model.onnx",
    export_params=True,
    pset_version=13,
    do_constant_folding=True,
    input_names=['input'],
    output_names=['db', 'mask'],
    dynamic_axes={
        'input': {0: 'batch_size', 2: 'height', 3: 'width'},
        'db': {0: 'batch_size', 2: 'height', 3: 'width'},
        'mask': {0: 'batch_size', 2: 'height', 3: 'width'}
    }
)
