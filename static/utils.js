const BASE_URL = "https://corsproxy.io/?url="
function httpGet(theUrl)
{
    var xmlHttp = new XMLHttpRequest();
    xmlHttp.open( "GET", `${BASE_URL}${encodeURIComponent(`https://word-of-day-2oouebir.fermyon.app${theUrl}`)}`, false ); // false for synchronous request
    xmlHttp.send( null );
    return JSON.parse(xmlHttp.responseText);
}
