type PagesType: cds.Integer;

entity Books {
  key ID: Integer;
  @myCustomAnnotation: 'foo'
  @myOtherCustomAnnotation: ['bar']
  title: String;
  author_ID: Integer;
  @anotherAnno: 'bar'
  @myArrayAnnotation: ['first', 'second']
  pages: PagesType;
}

entity Authors {
  key ID: Integer;
  firstName: String;
  lastName: String;
}
