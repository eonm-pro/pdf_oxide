#!/usr/bin/env python3
"""
OCR text extraction example using pdf_oxide.

This script demonstrates how to extract text from scanned PDFs using
PaddleOCR models via ONNX Runtime.

Prerequisites:
    1. Build pdf_oxide with OCR feature:
       maturin develop --features python,ocr

    2. Download PaddleOCR ONNX models:
       ./scripts/setup_ocr_models.sh
       Or manually download the recommended V4 det + V5 rec combination:
       - det.onnx   (ch_PP-OCRv4_det from https://huggingface.co/deepghs/paddleocr)
       - rec.onnx   (en_PP-OCRv5_mobile_rec from https://huggingface.co/monkt/paddleocr-onnx)
       - en_dict.txt (character dictionary, must have space as last line)

    3. ONNX Runtime (libonnxruntime.so v1.23+) must be on LD_LIBRARY_PATH.

Usage:
    python ocr_example.py <pdf_file> --det <det_model> --rec <rec_model> --dict <dict_file>

Example:
    python ocr_example.py scanned.pdf \
        --det models/det.onnx \
        --rec models/rec.onnx \
        --dict models/en_dict.txt
"""

import argparse
import sys
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description="Extract text from scanned PDFs using OCR")
    parser.add_argument("pdf", help="Path to PDF file")
    parser.add_argument("--det", required=True, help="Path to detection model (ONNX)")
    parser.add_argument("--rec", required=True, help="Path to recognition model (ONNX)")
    parser.add_argument("--dict", required=True, help="Path to character dictionary")
    parser.add_argument("--page", type=int, help="Process only this page (0-indexed)")
    args = parser.parse_args()

    # Import pdf_oxide
    try:
        from pdf_oxide import PdfDocument
    except ImportError as e:
        print(f"Error: Failed to import pdf_oxide: {e}")
        print("Make sure to build with: maturin develop --features python,ocr")
        sys.exit(1)

    # Import OCR classes (only available when built with 'ocr' feature)
    try:
        from pdf_oxide import OcrConfig, OcrEngine
    except ImportError:
        print("Error: pdf_oxide was not built with OCR support")
        print("Rebuild with: maturin develop --features python,ocr")
        sys.exit(1)

    # Validate paths
    if not Path(args.pdf).exists():
        print(f"Error: PDF file not found: {args.pdf}")
        sys.exit(1)
    for path, name in [
        (args.det, "detection model"),
        (args.rec, "recognition model"),
        (args.dict, "dictionary"),
    ]:
        if not Path(path).exists():
            print(f"Error: {name} not found: {path}")
            sys.exit(1)

    print("=" * 70)
    print("PDF OCR Example")
    print("=" * 70)
    print()

    # Create OCR configuration
    print("Configuring OCR engine...")
    config = OcrConfig(
        det_threshold=0.3,
        box_threshold=0.5,
        rec_threshold=0.5,
        num_threads=4,
    )
    print(f"Config: {config}")

    # Load OCR engine
    print("\nLoading OCR models...")
    try:
        engine = OcrEngine(
            det_model_path=args.det,
            rec_model_path=args.rec,
            dict_path=args.dict,
            config=config,
        )
        print("OCR engine loaded successfully!")
    except Exception as e:
        print(f"Error loading OCR engine: {e}")
        sys.exit(1)

    # Open PDF
    print(f"\nOpening PDF: {args.pdf}")
    try:
        doc = PdfDocument(args.pdf)
        page_count = doc.page_count()
        print(f"PDF has {page_count} pages")
    except Exception as e:
        print(f"Error opening PDF: {e}")
        sys.exit(1)

    # Determine pages to process
    if args.page is not None:
        if args.page < 0 or args.page >= page_count:
            print(f"Error: Page {args.page} out of range (0-{page_count - 1})")
            sys.exit(1)
        pages = [args.page]
    else:
        pages = range(page_count)

    # Process each page
    for page_idx in pages:
        print()
        print("-" * 70)
        print(f"Page {page_idx + 1} of {page_count}")
        print("-" * 70)

        try:
            text = doc.extract_text_ocr(page=page_idx, engine=engine)
            if text.strip():
                print("\nExtracted text:")
                print(text)
            else:
                print("(No text detected)")
        except Exception as e:
            print(f"OCR failed: {e}")

    print()
    print("=" * 70)
    print("Done!")
    print("=" * 70)


if __name__ == "__main__":
    main()
