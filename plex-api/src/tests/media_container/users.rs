use crate::media_container::UsersMediaContainer;
use quick_xml::de::from_str;

#[test]
fn decode_users() {
    let s = r##"
<?xml version="1.0" encoding="UTF-8"?>
<MediaContainer friendlyName="myPlex" identifier="com.plexapp.plugins.myplex" machineIdentifier="19A4B805E9021" totalSize="2" size="2">
    <User id="8676063" title="18A0A7BD14ECC" thumb="https://plex.tv/users/0d9d14488439289a/avatar?c=1536695183" protected="0" home="1" allowSync="0" allowCameraUpload="0" allowChannels="0" allowTuners="0" allowSubtitleAdmin="0" filterAll="" filterMovies="contentRating=G" filterMusic="" filterPhotos="" filterTelevision="contentRating=TV-14" restricted="1">
        <Server id="5558593" serverId="8035121" machineIdentifier="271663AA23AC6" name="LED-Kremen-286" lastSeenAt="1547660334" numLibraries="2" allLibraries="1" owned="1" pending="0"/>
    </User>
    <User id="7393808" title="2CD87DC316A42" thumb="https://plex.tv/users/80dab310d314a591/avatar?c=1536616777" protected="0" home="1" allowSync="0" allowCameraUpload="0" allowChannels="0" allowTuners="1" allowSubtitleAdmin="0" filterAll="" filterMovies="" filterMusic="" filterPhotos="" filterTelevision="" restricted="1">
        <Server id="717069" serverId="7142016" machineIdentifier="2A61C4586017F" name="plex-test-docker-e1491b81-445d-47c5-af76-4e0dcaf518dc" lastSeenAt="1547068479" numLibraries="4" allLibraries="1" owned="1" pending="0"/>
    </User>
    <User id="3609362" title="15FDD85D4BED2" thumb="https://plex.tv/users/80bab310d314a595/avatar?c=1536616777" protected="0" home="1" allowSync="0" allowCameraUpload="0" allowChannels="0" allowTuners="2" allowSubtitleAdmin="0" filterAll="" filterMovies="" filterMusic="" filterPhotos="" filterTelevision="" restricted="1">
    </User>
</MediaContainer>
    "##;

    let mc = from_str::<UsersMediaContainer>(s);
    assert!(mc.is_ok(), "Unable to deserialize users: {:?}", mc.err());
}
