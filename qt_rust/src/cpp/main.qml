import QtQuick 2.12
import QtQuick.Layouts 1.3
import QtQuick.Window 2.0
import RustCode 1.0;
import QtQuick.Controls 2.12
import QtQuick.Controls.Material 2.12;


ApplicationWindow {
    visible: true
    width: 600
    height: 800
    title: qsTr("File finder")

    Material.theme: Material.Dark
    Material.accent: Material.Purple

    header: ToolBar {
        Label {
            anchors.fill: parent
            text: qsTr("Qrep")
            font.pixelSize: 30
            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter
        }
    }


    Grep {
        id: grep
        query: query.text
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 10
        anchors.bottomMargin: 0

        // Button {
        //   text: "Test"
        // }

        RowLayout {
            Layout.fillWidth: true
            TextField {
                id: query
                Layout.fillWidth: true
                focus: true
                placeholderText: "Search"
                //                textColor: "#000000"
            }
            BusyIndicator {
                running: grep.busy
                visible: grep.busy
                Layout.preferredWidth: query.height
                Layout.preferredHeight: query.height
            }
        }

        ListView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            model: grep
            clip: true
            ScrollBar.vertical: ScrollBar {}

            delegate: Text {
                text: format(name, line)
                textFormat: Text.StyledText
                wrapMode: Text.WordWrap
                width: parent.width
                color: "#f0f0f0"
                MouseArea {
                    anchors.fill: parent
                    onClicked: Qt.openUrlExternally("file:" + path)
                }
            }
        }
    }
    function format(name, line) {
        let s = "<u><font color='#ffffff'>" + name + "</font></u>";
        if (line) {
            s += " " + line;
        }
        return s;
    }
}
