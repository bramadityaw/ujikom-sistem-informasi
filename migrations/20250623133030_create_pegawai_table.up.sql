create table if not exists pegawai
(
    nip            integer      not null primary key,
    nama           text         not null,
    alamat         text         not null,
    tanggal_lahir  text         not null,
    kode_divisi    text         not null,
    foreign key (kode_divisi) references divisi(kode)
);
