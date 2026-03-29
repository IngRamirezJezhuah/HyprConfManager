#!/bin/bash
#matamos cualquier instancia previa
pkill eww
sleep 0.2

#Definimos la ruta de nuestra config
CFG="$HOME/Documentos/Programacion/Proyectos-EWW/HyperConfManager/FrontEnd"

#abrimos la ventana apuntando a la carpeta Front
eww --config "$CFG" daemon &
sleep 0.5

#intentamos abir la ventana
eww --config "$CFG" open config-manager

