create table if not exists presensi
(
    tanggal text not null,
    nip integer not null,
    jam_masuk text,
    jam_pulang text,
    foreign key (nip) references pegawai(nip)
);
