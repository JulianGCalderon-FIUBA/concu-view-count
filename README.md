# Video Count by Channel - Parallel vs Sequential

## Dataset

Usa el dataset de [estadisticas de youtube](https://www.kaggle.com/datasets/datasnaek/youtube-new)

Los archivos `.csv` se deben extraer en una carpeta llamada `data` en la raiz del proyecto

## Implementaciones

La primer implementación es secuencial, usando iteradores estandares

La segunda es con iteradores concurrentes, a partir de la biblioteca Rayon


## Rendimiento

En mi computadora, el aumento de rendimiento es de 4x para la versión concurrente
