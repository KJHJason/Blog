use crate::templates::general::AwardInfo;

#[inline(always)]
pub fn get_awards<'a>() -> Vec<AwardInfo<'a>> {
    vec![
        AwardInfo{
            title: "Director's List for Year 3 Semester 2",
            issuer: "Nanyang Polytechnic",
            file_url: "https://drive.proton.me/urls/G2FC3XFW1W#TqIGyMLhGazS",
            date: "May 2024",
            img_src: "https://storage.kjhjason.com/images/awards/Y3_S2_201484K_KUAN JUN HAO JASON-1.webp",
            img_alt: "Year 3 Semester 2 Director's List Award Certificate",
        },
        AwardInfo{
            title: "Director's List for Year 3 Semester 1",
            issuer: "Nanyang Polytechnic",
            file_url: "https://drive.proton.me/urls/4HHNTA6QW8#LjWBNGMpLLRD",
            date: "Jan 2024",
            img_src: "https://storage.kjhjason.com/images/awards/Y3_S1_201484K_KUAN JUN HAO JASON-1.webp",
            img_alt: "Year 3 Semester 1 Director's List Award Certificate",
        },
        AwardInfo{
            title: "Director's List for Year 2 Semester 2",
            issuer: "Nanyang Polytechnic",
            file_url: "https://drive.proton.me/urls/4HHNTA6QW8#LjWBNGMpLLRD",
            date: "May 2023",
            img_src: "https://storage.kjhjason.com/images/awards/Y2_S2_SF2102_KUAN JUN HAO JASON_2022_S2-1.webp",
            img_alt: "Year 2 Semester 2 Director's List Award Certificate",
        },
        AwardInfo{
            title: "Director's List for Year 1 Semester 2",
            issuer: "Nanyang Polytechnic",
            file_url: "https://drive.proton.me/urls/4HHNTA6QW8#LjWBNGMpLLRD",
            date: "Jun 2022",
            img_src: "https://storage.kjhjason.com/images/awards/Y1_S2_ITDF13_2021S2_DirList1335-1.webp",
            img_alt: "Year 1 Semester 2 Director's List Award Certificate",
        },
    ]
}
