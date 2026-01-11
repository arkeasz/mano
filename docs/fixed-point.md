# Fixed-Point Arithmetic Library

> Documentación para el módulo de Fixed-Point Arithmetic.  
> Esta librería permite realizar operaciones aritméticas con números de punto fijo, evitando errores de precisión de los flotantes en aplicaciones críticas.

---

## Índice

- [Descripción](#descripción)  
- [Características](#características)  
- [Instalación](#instalación)  
- [Tipos de datos](#tipos-de-datos)  
- [Funciones disponibles](#funciones-disponibles)  
- [Ejemplos](#ejemplos)  
- [Notas importantes](#notas-importantes)  
- [Pruebas](#pruebas)  
- [Licencia](#licencia)  

---

## Descripción

El módulo de **fixed-point** permite representar números decimales usando enteros y un factor de escala fijo, lo que garantiza operaciones deterministas y sin pérdida de precisión de punto flotante.  
Ideal para sistemas embebidos, finanzas, y cálculos que requieren alta exactitud.

---

## Características

- Soporte para operaciones básicas: suma, resta, multiplicación y división.  
- Conversión a y desde flotantes y enteros.  
- Configurable: número de bits para la parte entera y fraccionaria.  
- Funciones auxiliares: comparación, truncado, redondeo.  

---

## Instalación

```bash
# Clona el repositorio
git clone https://github.com/tuusuario/fixed-point.git

# O si es un paquete (ejemplo Rust)
cargo add fixed-point
