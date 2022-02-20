entity Books {
  key ID: Integer;
  @myCustomAnnotation: 'foo'
  @myOtherCustomAnnotation: ['bar']
  title: String;
  author_ID: Integer;
}

entity Authors {
  key ID: Integer;
  firstName: String;
  lastName: String;
}
