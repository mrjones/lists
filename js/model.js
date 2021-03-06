/* @flow */

var api = require('./api_pb.js');

function user_from_json(json: Object): api.User {
  var user = new api.User();
  user.setName(json.name);
  user.setId(json.id);
  return user;
}

class StreetEasyAnnotation {
  hash: string;
  name: string;
  price_usd: number;
  open_houses: string[];

  static from_json(json: Object) {
    var annotation = new StreetEasyAnnotation();
    annotation.hash = json.hash;
    annotation.name = "flow:" + json.name;
    annotation.price_usd = json.price_usd;
    annotation.open_houses = json.open_houses;
    return annotation;
  }
}

class ListItem {
  id: number;
  name: string;
  description: string;

  text_annotations: string[];
  link_annotations: string[];
  streeteasy_annotations: StreetEasyAnnotation[];

  static from_json(json_item: Object) {
    var item = new ListItem();
    item.name = json_item.name;
    item.description = json_item.description;
    item.text_annotations = json_item.text_annotations;
    item.link_annotations = json_item.link_annotations;
    item.streeteasy_annotations = json_item.streeteasy_annotations.map(
      StreetEasyAnnotation.from_json);
    return item;
  }
}

class List {
  name: string;
  items: ListItem[];

  static from_json(json: Object) {
    var list = new List();
    list.name = json.name;
    list.items = json.items.map(ListItem.from_json);
    return list;
  }
}

module.exports = {
  user_from_json,
  
  List,
  ListItem,
  StreetEasyAnnotation,
};
