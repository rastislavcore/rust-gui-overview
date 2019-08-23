#include "Bindings.h"
#include <QtGui/QGuiApplication>
#include <QtQml/QQmlApplicationEngine>
#include <QQuickStyle>
#include <QtCore/QtPlugin>

// static
#ifdef _QT_STATIC
    // https://doc.qt.io/qt-5/plugins-howto.html#static-plugins
    #ifdef _WIN32
        Q_IMPORT_PLUGIN(QWindowsIntegrationPlugin)
        Q_IMPORT_PLUGIN(QtQuick2DialogsPrivatePlugin)
        Q_IMPORT_PLUGIN(QmlFolderListModelPlugin)
    #elif __linux__
        Q_IMPORT_PLUGIN(QXcbIntegrationPlugin)
        Q_IMPORT_PLUGIN(QXcbGlxIntegrationPlugin)
        Q_IMPORT_PLUGIN(QEvdevKeyboardPlugin)
        Q_IMPORT_PLUGIN(QEvdevMousePlugin)
    #else // MacOS
        Q_IMPORT_PLUGIN(QCocoaIntegrationPlugin)

    #endif

    Q_IMPORT_PLUGIN(QtQuick2Plugin)
    Q_IMPORT_PLUGIN(QtQuickControls2Plugin)
    Q_IMPORT_PLUGIN(QtQuickLayoutsPlugin)
    Q_IMPORT_PLUGIN(QtQuick2WindowPlugin)
    Q_IMPORT_PLUGIN(QtQuickControls2MaterialStylePlugin)
    Q_IMPORT_PLUGIN(QtQuickTemplates2Plugin)
//    Q_IMPORT_PLUGIN(QmlSettingsPlugin)
//    Q_IMPORT_PLUGIN(QmlSettingsPlugin)
//    Q_IMPORT_PLUGIN(QtQuick2PrivateWidgetsPlugin)
#endif

extern "C" {
    int main_cpp(const char* appPath);
}

int main_cpp(const char* appPath) {
    Q_INIT_RESOURCE(qml); // https://doc.qt.io/qt-5/resources.html
    QQuickStyle::setStyle("Material");

    int argc = 1;
    char* argv[1] = { const_cast<char*>(appPath) };
    QGuiApplication app(argc, argv);

    qmlRegisterType<Grep>("RustCode", 1, 0, "Grep");
    QQmlApplicationEngine engine;
    engine.load(QUrl(QStringLiteral("qrc:/main.qml")));
    return app.exec();
}
