TEMPLATE  = lib
CONFIG    += staticlib c++11 qt static
HEADERS   = Bindings.h
SOURCES   = main.cpp \
            Bindings.cpp
RESOURCES = qml.qrc

QT += widgets qml quick quickcontrols2
requires(qtConfig(combobox))

CONFIG += qtquickcompiler

QTPLUGIN     += qtquick2plugin qtquickcontrols2plugin qtquick2dialogsplugin qmlsettingsplugin qtquickcontrols2materialstyleplugin qtquicktemplates2plugin
QTPLUGIN.platforms += qcocoa
