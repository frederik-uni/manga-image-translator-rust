import onnx

model = onnx.load("./inference/rec.onnx")

with open("./inference/ppocr_keys_v1.txt", "r", encoding="utf-8") as f:
    keys_content = f.read()
# Create metadata entry
metadata_prop = onnx.StringStringEntryProto()
metadata_prop.key = "character"
metadata_prop.value = keys_content

# Add it to the model
model.metadata_props.append(metadata_prop)

onnx.save(model, "./inference/rec_with_metadata.onnx")
