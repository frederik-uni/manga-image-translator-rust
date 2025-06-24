#!/bin/bash

python3 -m venv venv
source venv/bin/activate

pip install PaddlePaddle==3.0.0b2
pip install onnxruntime paddle2onnx onnx

wget -nc -P ./inference https://paddleocr.bj.bcebos.com/PP-OCRv4/chinese/ch_PP-OCRv4_det_server_infer.tar
cd ./inference && tar xf ch_PP-OCRv4_det_server_infer.tar && cd ..

wget -nc  -P ./inference https://paddleocr.bj.bcebos.com/PP-OCRv4/chinese/ch_PP-OCRv4_rec_infer.tar
cd ./inference && tar xf ch_PP-OCRv4_rec_infer.tar && cd ..

wget -nc  -P ./inference https://paddleocr.bj.bcebos.com/dygraph_v2.0/ch/ch_ppocr_mobile_v2.0_cls_infer.tar
cd ./inference && tar xf ch_ppocr_mobile_v2.0_cls_infer.tar && cd ..

wget -nc -P ./inference \
  https://raw.githubusercontent.com/PaddlePaddle/PaddleOCR/release/3.0/ppocr/utils/ppocr_keys_v1.txt

paddle2onnx --model_dir ./inference/ch_PP-OCRv4_det_server_infer \
--model_filename inference.pdmodel \
--params_filename inference.pdiparams \
--save_file ./inference/det.onnx \
--opset_version 11 \
--enable_onnx_checker True

paddle2onnx --model_dir ./inference/ch_PP-OCRv4_rec_infer \
--model_filename inference.pdmodel \
--params_filename inference.pdiparams \
--save_file ./inference/rec.onnx \
--opset_version 11 \
--enable_onnx_checker True

paddle2onnx --model_dir ./inference/ch_ppocr_mobile_v2.0_cls_infer \
--model_filename inference.pdmodel \
--params_filename inference.pdiparams \
--save_file ./inference/cls.onnx \
--opset_version 11 \
--enable_onnx_checker True

python3 add_metadata_to_onnx.py
