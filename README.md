# Aplicación TFT LED ESP32

Este proyecto es una aplicación basada en Rust diseñada para un microcontrolador ESP32 para mostrar imágenes en una pantalla TFT LCD (ILI9341). Inicializa el ESP32, configura la interfaz SPI para la comunicación con la pantalla y luego muestra una imagen.

## Características

* **Basado en ESP32**: Utiliza las capacidades del microcontrolador ESP32.
* **Lenguaje de Programación Rust**: Desarrollado usando Rust para sistemas embebidos, aprovechando la biblioteca `esp-hal`.
* **Pantalla TFT ILI9341**: Se interconecta con el popular controlador de LCD TFT ILI9341.
* **Visualización de Imágenes**: Capaz de mostrar datos de imagen crudos (formato RGB565) en la pantalla.
* **Comunicación SPI**: Usa SPI para la comunicación entre el ESP32 y la pantalla TFT.
* **Registro Configurable**: Incluye soporte para el registro de mensajes, configurable mediante variables de entorno.

## Requisitos de Hardware

* Microcontrolador ESP32
* Pantalla TFT LCD ILI9341 (240x320)
* Conexiones para SPI (SCK, MOSI, MISO, CS), DC (Datos/Comando), Reset y LED. La configuración actual de pines en `main.rs` es:
    * SCK: GPIO18
    * MOSI: GPIO23
    * MISO: GPIO19
    * CS: GPIO5
    * DC: GPIO2
    * Reset: GPIO4
    * LED: GPIO21

## Software y Configuración

El proyecto está construido usando Rust y el framework `esp-idf`. Las dependencias clave incluyen:
* `esp-hal` para la abstracción de hardware del ESP32.
* El crate `ili9341` para el controlador de la pantalla.
* `display-interface-spi` para la interfaz de pantalla SPI.
* `embedded-graphics` para dibujar.

El archivo `template.yaml` describe varias opciones de configuración para el proyecto, incluyendo la habilitación de características HAL inestables, asignación de memoria, Wi-Fi, BLE, soporte para el framework Embassy y diferentes herramientas de flasheo/depuración como `probe-rs` o `espflash`.

### Creación de Imágenes Raw Big Endian (BE) con FFmpeg

Para mostrar imágenes personalizadas, necesitan ser convertidas al formato crudo RGB565 Big Endian. Esto se puede lograr utilizando ImageMagick para redimensionar y convertir al formato PPM, y luego dirigir esa salida a FFmpeg mediante una tubería (pipe).

Un comando útil para esta conversión es:

```bash
convert nombre_imagen.jpeg -resize 240x320\! ppm:- | \
ffmpeg -f image2pipe -vcodec ppm -i - -f rawvideo -pix_fmt rgb565be output.raw