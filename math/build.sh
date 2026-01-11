#!/usr/bin/env bash

cd "$(dirname "$0")" || exit 1

echo "Compilando librería compartida a partir de los archivos de math..."

gfortran -shared -ffree-line-length-none \
  -Wall -Wextra \
    utils.f90 colors.f90 tokens.f90  symtab.f90 parser.f90 ast.f90 math.f90 eval.f90 calculator.f90 calc.f90 \
  -o libmath.so

if [ -f libmath.so ]; then
  echo "Librería generada exitosamente: libmath.so"
else
  echo "ERROR: No se generó la librería."
  exit 1
fi

read -rp "Presiona ENTER para salir..."
